# Background

Pacenotes are used by rally drivers, as a way to see into the future. They are produced by the co-drivers to enable their partner driver to be ready for the upcoming obstacles throughout the rally stage.

# Research

## Directions APIs

- Google maps
- [Open Source Routing Machine](http://project-osrm.org)

# Problem / Todo

The below list details the tasks this project must accomplish in order to be deemed a success. However, please note that some of these tasks are unnecessary for the MVP. The further sub-paragraphs will detail the research, problems and thus solutions of each task.

- [ ] Location of points within world
- [ ] Angle of corner
- [ ] Length of corner (radius of curve)
- [ ] Elevation +/-
- [ ] Potential obstacles along stage (this is difficult)

## Location of points within world

To calculate all necessary components required to compute on-the-fly pacenotes we must computationally establish a world coordinate space.
An obvious solution to this would be Longitude and Latitude; the bounds of which are:

**Longitude:** -180 - 180

**Latitude:** -90 - 90

Luckily all of the aforementioned direction APIs also work using Longitude and Latitude.
Rust provides 2 floating point number data types: `f32` and `f64`; of which the former will be capable of storing the most precise/longest number; 13.388782, for example.

## Angle of corner

I would like to implement both methods of calculating angles of a corner: bearings and triangles, and then run tests to decide which function/method is most appropriate.

### Triangles

$$
Angle \theta = \cos(\frac{b^2 + c^2 - a^2}{2bc})
$$

### Bearings

### Optimisations

Check if the triangles right angle?

## Length of corner (radius of curve)

## Elevation +/-

## Potential obstacles along stage (this is difficult)

# Similar projects

## geordanr/pacenotes

Link: https://github.com/geordanr/pacenotes

An 8 year old project written in CoffeeScript. Considers the elevation, angle and corner length. All parsed from the Google Maps API provided JSON.

## vishaalprasadsjsu/pacenotes-old

Link: https://github.com/vishaalprasadsjsu/pacenotes-old

A 5 year old project written in Java for an Android app target.
