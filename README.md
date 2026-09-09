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

Currently work in progress, all code that exists is covered by tests, but most is not hooked
up to the main CLI yet.