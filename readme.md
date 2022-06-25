# Neon White ghost parser

This tool pretty much just reads in a .phant file (stored in `Users/{username}/AppData/LocalLow/Little\ Flag\ Software,\ LLC/Neon\ White/{steam user id})/Ghosts`) and parses it into a rust representation.

Currently there's not much use for this beyond having a reasonably human-readable representation of Neon White ghost files - my aim is to eventually compute some stats on the ghosts to have an indication of if a .phant file is legitimate or not.

## Phant files

Whenever you set a PB, Neon White saves that run to `0.phant` in `Ghosts/{Level Name}/`. Ghost recording is not active in 'boss fight' levels or in Hell Rushes, but does seem to be active in Heaven Rush (however, I don't think it's actually used in heaven rush? Still need to poke at what happens there).

Only your most recent PB is stored this way. There's support for phant files to use different numbers (possibly was supposed to be a cache of other players' ghosts?); you should probably rename your phant to `{level name}_{steam user id}.phant` before sending it to someone to avoid clobbering ghost files.

## Limitations

The ghost files store the overall time the ghost took, as well as a bunch of captured 'frames' with associated timestamps. These frames are only really snapshots at about 30 ticks [note: verify, this is an assumption] of the position, camera angle and direction, and action states of the ghost (e.g. grappling, stomping, discarding, shooting) at that tick.

The ghost files do _not_ record inputs or velocities - while velocities _can_ sorta be extrapolated from these replays by comparing positions (e.g. to see if someone's godspeed was modded to be 10% faster), there's no way to grab the inputs to re-run them locally or anything of the sort.