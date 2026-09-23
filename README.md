# Precise Isovist

This is an attempt to write a modern isovist analysis tool

## Goals

The goals for this tool are:

- precise calculation - using ray casting and floating numbers to calculate isovists at
  arbitrary/sytem defined precision rather than on a grid
- fast calculations - using rust to write a fast calculation library
- everything reachable via a CLI tool, to allow automation for working with large data sets

The last point means that visualisation is secondary concern in this project - initially a
everything will be done based of DXF files for plans/spactial data, so a tool capapble of 
viewing or editing DXF files is required

## Current status

Currently, there is a simple main that can calculate isovists around points marked in a dxf
file.
It will read in a dxf file and expects the plan to be analysed in one layer, and the centre
of the isoisvists in a separte layer. Both layers have to be specified on the command line.
The program will calculate a 360 degree isovist around all points, write them to separate
layers in the DXF file and save it again as ouput file.

There is sample plan in `./test_data/gallery_isovistpoint.dxf`, with the plan on layer `0`
and a single isovist point on layer `isovist_points`.

The sample command line to run this would be 
```
cargo run -- -f test_data/gallery_isovistpoint.dxf -i isovist_points -p 0 -o output.dxf
```