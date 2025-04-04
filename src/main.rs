use std::collections::HashMap;
use std::time::SystemTime;

use crate::containers::FwdStrandSplitReadSegment;
use cli::{get_args, Arguments};
use containers::{ComplexSVCalls, Connection, Coordinate, TargetCoordinate};
use log::{debug, error, info, LevelFilter};
use std::env;
use utils::is_local_file;

mod bam_sa_parser;
mod block_filter;
mod cli;
pub mod cluster_connector;
pub mod cluster_finder;
mod containers;
pub mod event_graph_builder;
mod graph_annotator;
mod ingester;
pub mod result_writer;
mod utils;

fn set_up() -> (Arguments, Option<TargetCoordinate>) {
    let args = get_args();
    let filter_level: LevelFilter = match args.verbose {
        false => LevelFilter::Info,
        true => LevelFilter::Debug,
    };
    env_logger::builder()
        .format_timestamp_millis()
        .filter_level(filter_level)
        .init();

    let version = env!("CARGO_PKG_VERSION");
    info!("\nRunning HiFi-SVTopo v{}\n", version);

    let cmd: Vec<String> = env::args().collect();
    let cmd_str = cmd.join(" ");
    debug!("Run command: {}", cmd_str);
    debug!("v{}\n", version);

    let mut has_vcf = false;
    let mut has_json = false;
    if args.vcf_filename.is_some() {
        has_vcf = true;
    }
    if args.json_filename.is_some() {
        has_json = true;
    }
    if has_json && !has_vcf {
        error!("`--variant-readnames` json input requires `--vcf` input");
        std::process::exit(exitcode::CONFIG);
    }
    if !has_json && has_vcf {
        error!("`--vcf` input requires `--variant-readnames` json input");
        std::process::exit(exitcode::CONFIG);
    }

    if let Some(region_str) = args.target_region.clone() {
        let target_region = TargetCoordinate::new(region_str);
        return (args, Some(target_region));
    }

    if !is_local_file(&args.exclude_regions_path) {
        error!(
            "Exclude regions file {} not found",
            args.exclude_regions_path
        );
        std::process::exit(exitcode::CONFIG);
    }

    let path = std::path::Path::new(&args.outdir);
    if !path.exists() || !path.is_dir() {
        error!("outdir {} does not exist", args.outdir,);
        std::process::exit(exitcode::CONFIG);
    }
    if args.prefix.contains('_') {
        error!("Prefix does not allow underscores");
        std::process::exit(exitcode::CONFIG);
    }

    (args, None)
}

fn log_time(start_time: SystemTime) {
    let elapsed_time = start_time.elapsed().unwrap().as_secs();
    let hours = elapsed_time / 3600;
    let minutes = (elapsed_time % 3600) / 60;
    let seconds = elapsed_time % 60;
    debug!("Running time: {}h:{}m:{}s", hours, minutes, seconds);
}

fn main() {
    ///////////////////////////////////////////////////////////////////////////
    // Set up
    let (args, target_opt) = set_up();
    let start_time = SystemTime::now();
    let mut exclude_regions = ingester::load_exclude_regions(args.exclude_regions_path);
    let vcf_coord_map: HashMap<String, Vec<Coordinate>>;

    ///////////////////////////////////////////////////////////////////////////
    // Get read data
    let clipped_reads: HashMap<String, Vec<FwdStrandSplitReadSegment>>;
    if let Some(target_coord) = target_opt.clone() {
        clipped_reads =
            ingester::get_split_alignments_from_region(args.bam_filename.clone(), target_coord);
    } else {
        clipped_reads =
            ingester::get_split_alignments(args.bam_filename.clone(), &mut exclude_regions);
    }

    ///////////////////////////////////////////////////////////////////////////
    // Get variant ID maps to readnames and vcf breakpoints as connections
    // if available.
    let (clip_coordinates, vcf_connections) = if let (Some(vcf_filename), Some(json_filename)) =
        (args.vcf_filename, args.json_filename)
    {
        let sample_id: String = ingester::get_sample_from_bam(args.bam_filename.clone());

        let vcf_breaks: (Vec<Connection>, HashMap<String, Vec<Coordinate>>) =
            ingester::get_vcf_breaks(vcf_filename, &exclude_regions, &target_opt);
        let variant_ids: std::collections::HashSet<String> = vcf_breaks.1.keys().cloned().collect();
        let variant_readnames: HashMap<String, Vec<String>> =
            ingester::get_read_info_from_json(json_filename, sample_id, variant_ids);
        vcf_coord_map = vcf_breaks.1;

        // Find genomic break locations using VCF break locations if provided,
        (
            cluster_finder::assign_clipped_reads_to_clusters(
                &clipped_reads,
                &vcf_coord_map,
                &variant_readnames,
            ),
            vcf_breaks.0,
        )
    } else {
        // Find genomic break locations using clustered groups of clipped reads
        (
            cluster_finder::find_breaks(&clipped_reads, args.allow_unphased),
            Vec::new(),
        )
    };

    // Connect genomic break locations using connections from the VCF, alignments, & phasing
    let break_connections: HashMap<Connection, Vec<FwdStrandSplitReadSegment>> =
        cluster_connector::connect_clusters(&clip_coordinates, &vcf_connections, args.allow_unphased);

    ///////////////////////////////////////////////////////////////////////////
    // Join 1-to-1 connections into full connected event graphs
    let event_graphs: Vec<containers::EventGraph> =
        event_graph_builder::build_event_graphs(&break_connections, &clip_coordinates);

    ///////////////////////////////////////////////////////////////////////////
    //Annotate event graphs with ordering info and directionality
    let annotated_graphs: ComplexSVCalls =
        graph_annotator::annotate_graphs(&event_graphs, &clip_coordinates);

    // Filter/write results
    ///////////////////////////////////////////////////////////////////////////
    let mut filtered_annotated_graphs: ComplexSVCalls;
    if args.nofilter || target_opt.is_some() {
        filtered_annotated_graphs = annotated_graphs;
    } else {
        filtered_annotated_graphs =
            block_filter::apply_filters(&annotated_graphs, args.bam_filename, args.max_coverage);
    }
    filtered_annotated_graphs.event_graphs.sort_unstable();
    result_writer::write_results(
        filtered_annotated_graphs,
        args.outdir.trim_end_matches('/').to_string(),
        args.prefix,
        args.write_unzipped,
    );
    log_time(start_time);
}
