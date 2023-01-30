

# Ticker



# Char



# Random

Need to pick a good pseudo-random number generator state.




# Graphics

- 64x64 bitmap display
- 8x8 sprites, can fit 64 on a screen
- each sprite has a 6-bit id. The upper 2 bits represent whether to flip the sprite horizontally and vertically




x
    write: x coordinate
    read: x coordinate
y
    write: y coordinate
    read: y coordinate

draw
    write: draw current frame buffer, don't clear, stall for next frame

flip
    write: wait for next frame, draw and clear frame buffer

cls

sprite
    write: draw sprite at (x,y)
    read: sprite id

color
    write: set color pallette
    read: get color pallette
    notes: 4-bit color pallette https://romanzolotarev.com/pico-8-color-palette/


how to implement scrolling without redrawing screen the whole time?

links
- https://www.youtube.com/@docrobs/videos

