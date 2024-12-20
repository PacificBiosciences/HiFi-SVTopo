# Result interpretation

## Image components
`svtopo` produces publication-quality images of the genomic blocks altered in structural rearrangements. Genomic breaks are identified relative to the reference genome by identifying clusters of split or clipped alignments. SV plots may include these components:
* _Reference chain plot_, meaning a chained plot of genomic blocks in the reference genome, shown in reference order. Blocks begin at the start of the leftmost chimeric alignment supporting a breakend in the complex SV and transition at each subsequent breakend, culminating at the end of the rightmost chimeric alignment.
* _Spanned blocks_ are blocks where reads are aligned to the reference genome. These blocks are each softclipped at one or both ends. The thickness/weight of the lines for the spanned blocks indicates the number of clipped alignments in each position. Spanned blocks are horizontal groups of dark lines inside a black box. The block weights are not intended to represent the exact number of alignments present, but to give an overview of relative chimeric alignment support across the block.
* _Unspanned blocks_ are dashed lines connecting the end of a spanned block to the end of the next spanned block, with order determined by chimeric alignment order from the originating long read.
* _Alternate chain plots_, one or two chained plots of genomic blocks in sample order, representing possible alternate haplotype structures and shown in context of a second copy of the reference genome structure chain plot. Arrowhead directions indicate alignment orientation relative to the reference genome. Missing blocks represent deletions and repeated blocks indicate duplications. Blocks size is not maintained between this plot and the reference chain plot at the top.

## Gallery of examples
* [Complex SV example](#complex-sv)
* [Double deletions](#double-deletions)
* [Inversion with flanking deletions](#inversion-with-flanking-deletions)
* [Inverted non-tandem duplication followed by deletion](#inverted-non-tandem-duplication-followed-by-deletion)
* [Deletion followed by inverted non-tandem duplication and deletion](#deletion-followed-by-inverted-non-tandem-duplication-and-deletion)
* [Balanced inverted translocation](#balanced-inverted-translocation)

### Complex SV:
![system of deletions and inversions_example](imgs/complex_fully_connected.png)
This image is a representation of a complex SV consisting of genomic blocks A-J, where:
* A, C, and J are unchanged
* E and G are inverted
* B, D, F, and H are deleted

The sizes of the blocks are annotated in the legend on the right. The order and orientation of the sample genome relative to the reference are shown by the chain plot at the bottom, where the order of A, E, C, G, and J is shown with arrows indicating inversion of E and G.
Optional gene annotations appear at the bottom of the main plot window, indicating olfactory recepter gene overlaps.

IGV of the same region for comparison, with supplementary alignments linked:
![igv_comparison](imgs/igv/igv_complex_sv.png)

<br><br>

### Double-deletions

![adjacent_dels](imgs/simple_double_del.png)
This example contains two deletions B and D, separated by a small (61 bp) conserved region C.

IGV of the same region for comparison, with supplementary alignments linked:
![igv_comparison](imgs/igv/igv_double_deletion_1.png)

<br><br>

![two_dels_with](imgs/two_dels.png)
This example contains two deletion events that are farther apart (~75 kbp) but phased to the same haplotype.

IGV of the same region for comparison, with supplementary alignments linked:
![igv_comparison](imgs/igv/igv_double_deletion_2.png)

<br><br>

### Inversion with flanking deletions
![two_dels_with_inv](imgs/two_dels_with_inv.png)
This example also contains two deletion events, but in this case the non-deleted region between them is also inverted (identifiable from the dashed lines). The inversion is also visible in the `Sample structure` chain plot at the bottom.

IGV of the same region for comparison, with supplementary alignments linked:
![igv_comparison](imgs/igv/igv_inversion_with_flanking_deletions.png)

<br><br>

### Inverted non-tandem duplication followed by deletion
![inv_dup_and_del](imgs/inverted_dup_and_del.png)
In this example, after an initial A->B->C structure, there is a second copy of B in inverted orientation. The second copy is immediately followed by region E, which means the region D between them is omitted. This example thus contains an inverted non-tandem duplication and a deletion.

IGV of the same region for comparison, with supplementary alignments linked:
![igv_comparison](imgs/igv/igv_nontandem_duplication_followed_by_deletion.png)

<br><br>

### Deletion followed by inverted non-tandem duplication and deletion
![del_inv_dup](imgs/del_inv_dup.png)
Similarly this example contains a deletion and an inverted non-tandem duplication. The order is changed from the previous example as the first rearrangement is the deletion.

IGV of the same region for comparison, with supplementary alignments linked:
![igv_comparison](imgs/igv/igv_deletion_followed_by_inverted_non_tandem_duplication_and_deletion.png)

<br><br>

### Balanced inverted translocation
![balanced-inv](imgs/translocation.png)
This example shows an inverted translocation of a 3 kbp sequence from chr12 to chr10. The double line in the `Reference path` chain plot shows the chromosomal transition.

IGV of the same region for comparison, with supplementary alignments linked:
![igv_comparison](imgs/igv/igv_balanced_inverted_translocation.png)
