# SVTopo User Guide

## Table of Contents
* [Overview](#overview)
* [Getting Started](#getting-started)
* [Outputs](#outputs)
* [Usage Details](#usage-details)

## Overview

SVTopo creates images to represent chimeric/split alignment evidence for structural variation, with an emphasis on complex variants. For the purposes of SVTopo, these are defined as SVs large enough to for the PBMM2 aligner to align reads in in multiple chimeric alignments. Novel insertions are not supported and by default simple deletions and duplications are omitted from results. Complex SVs in SVTopo therefore consist of inversions, translocations, and multi-breakend combinations of simple rearrangements.

Processing steps:
1. Extract reads flagged with the BAM split alignment tag (`SA`)
2. Identify coordinates where alignment clipping sites from multiple reads are clustered
3. Connect pairs of coordinates that are joined by alignments
4. Further connect these pairs to generate graphs of breakends that define structural rearrangements
5. Emit graph representations as a JSON file

## Getting started 

### Installation

#### Install from GitHub
The `svtopo` Rust utility is available from the [Releases](https://github.com/PacificBiosciences/HiFi-SVTopo/releases) page.

It can be downloaded and run directly on Linux systems, as described in the **Quick Start** section of [HiFi-SVTopo usage](svtopo_usage.md)

The `svtopovz` utility can be downloaded by cloning this reposity with git or by downloading the `Source code (zip)` or `Source code (tar.gz)` assets from the [Releases](https://github.com/PacificBiosciences/HiFi-SVTopo/releases) page.

It can then be installed from source as shown below. It is recommended that this tool be installed in a fresh [Conda](https://conda.io/projects/conda/en/latest/index.html) environment with Python=3.10. 

Install from source, then test that the installation succeeded:
```bash
git clone https://github.com/PacificBiosciences/HiFi-SVTopo.git
cd HiFi-SVTopo/HiFi-SVTopoVz/
python setup.py install
svtopovz -h
```
If successfully installed, this will print out the tool version and command-line options.

#### Install from Conda
_Coming soon_

### Analysis Steps
SVTopo finds and plots SV images in two steps:
1. `svtopo` - Parses the phased BAM, exclude regions BED, and optional sawfish variant call files (VCF and `supporting_reads.json`) to create a complex variant graph representation in JSON format.
2. `svtopovz` - Reads the svtopo output JSON file and uses the complex variant graph information to create plots of the complex variants.

### Example workflow
To plot complex SVs, run SVTopo on the mapped and phased sample BAM, and then run SVTopoVz on the resulting JSON file.

The following example shows how this is done for a sample BAM called `HG002_hg38.bam` with sawfish VCF and `supporting_reads.json` inputs. The sawfish inputs may be omitted, but their inclusion improves results.

```bash
svtopo\
    --bam HG002_hg38.bam\
    --json-out svtopo_hg002_output.json.gz\
    --variant-readnames sawfish_HG002_supporting_reads.json\
    --vcf HG002_sawfish.vcf.gz \
    --exclude-regions https://github.com/PacificBiosciences/HiFiCNV/raw/refs/heads/main/data/excluded_regions/cnv.excluded_regions.hg38.bed.gz
mkdir images/
svtopovz \
  --json svtopo_hg002_output.json.gz\
  --out-prefix images/hg002_svtopo
```
This case (HG002 30X) generated 202 complex SV images, increasing to 626 when the `--include-simple-dels` and `--include-simple-dups` options were added to the SVTopoVz step.

## Outputs
### SVTopo step
The primary output of the SVTopo step is the JSON file detailing the complex SVs found. This may be viewed by the user directly but is not optimized for human readability.

### SVTopoVz step
The SVTopoVz step generates images for each of the entries in the complex SV JSON file that:
* are not simple deletions or tandem duplications
* can be fully resolved by SVTopoVz into a complex SV graph

For detailed notes on interpretation of these plots, see [result interpretation](https://github.com/PacificBiosciences/HiFi-SVTopo/blob/main/docs/result_interpretation.md).

## Usage details
### Expected compute requirements
SVTopo benchmarks with a 30x HiFi genome, with and without the recommended sawfish-based option:

__Sawfish benchmark:__
* SVTopo
  * Runtime 13 mins:33 secs
  * 3.3 GB RAM
* SVTopoVz
  * 1 min:1 sec
  * 0.6 GB RAM
  * 202 complex SV images

__Non-sawfish benchmark:__
* SVTopo
  * Runtime 10mins:2secs
  * 3.4 GB RAM
* SVTopoVz
  * 1 min:2 secs
  * 0.6 GB RAM 
  * 216 complex SV images
  
### Algorithm notes
* Clipped alignments: SVTopo uses chimeric/split alignments to identify signals of structural variation. These are defined as alignments with at least 100 bases of soft-clipping on either end of the alignment. Alignments with MAPQ < 20 are omitted.
* Finding break clusters with alignments: Locations of genomic breaks are identified using alignment clipping locations that are clustered together. These must be within a 10 bp confidence interval of each other, allowing for small differences of alignment and minimal sequencing error. Break cluster positions are given from the middle of the clip sites among alignments within 10 bp.
* Finding break clusters with sawfish output: If a VCF and `supporting_reads.json` are provided from [sawfish](https://github.com/PacificBiosciences/sawfish), these are used to identify additional break locations (the VCF SV POS and END locations). In this case, the alignments from a read assigned to a given variant ID in the sawfish output are compared to the variant breakend locations and the clipping sites nearest the variant break locations are used to assign specific split alignments from the read to the coordinate of the breakend.
* Phase filter: break clusters are only included if the phase is unambiguous. That is defined as having exactly one haplotype ID and one phaseset ID, with at least 2 phased alignments. Unphased alignments may also be included.
* Break connections: Once breaks are identified, they can be connected in pairwise fashion by alignments that are shared between them, or by using VCF entry connections (via VCF record POS/END). Alignment-based connections must have a minimum of two such shared alignments and break connections must be within 1 mb of each other (if on the same chromosome).
* Phased connection of clusters: In some cases phasing is used to connect break locations instead of directly using spanning alignments. In these cases, the clusters must be within 500kb of each other, have the same phaseset ID, and be on the same haplotype.
* Ambiguous sample order: In some cases it may be impossible to determine the order of some genomic blocks. These are given the sample sample_order_index entry in JSON and plotted in images with red outlines.
* The following filters are applied:
  * Coverage of 300x or less (can be changed using the `max-coverage` option)
  * No more than 5% of reads below MAPQ of 5