CRB alarm 

# first sweep 
- [X] integrate clap with timeout 
- [X] have a notification when timeout
- [X] display time in hms 
- [X] play music with rodio(or else) at timeout


# what's next
after the first inital working skeleton, that I call the first sweep. just having the basic.
These are the details of idea of where it can go next


## the animation
- [X] animation zigzag
- [X] animation archimedean spiral
- [ ] animation change depending on duration(long animation for long timeout)
- [ ] overlay progression have a gray full animation to indicate progression
- [ ] for long timeout make appropriate animation, have long vs short animation selection
- [ ] have numbers symbols large for timeout
- [ ] better timeout displaying
- [X] select random animation
- [X] select random marker that match the best


## music player
- [ ] how to integrate spotify api or something play with a given time?
- [ ] start music
- [ ] stop music
- [ ] increase volume as timeout reaching 0 
- [ ] stop music when timeout increase after reach 0
 
 ## control
- [ ] have a start stop key binding
- [ ] have a option, arg to play music during time or after time
- [ ] play crb snipped on some control 
- [ ] better increase /decrese timeout that sync better with animation


## the fucking github workflow..
- [ ] workflow with git release a version.. I don't what to do with this mess

## genral improvement and things
- [ ] animation circle sucks with large timeout
- [ ] in general the application is slow less than 30fps(on my shitty laptop though)
- [ ] write the readme and explain 
- [ ] not stable, it crash when increase/reduce time
- [ ] test on different os/platform look like audio is not working smooth on all platform

## bug to fix maybe
- [ ] zig zag is glitchy pattern is off the screen

## research
- [ ] spotify api how it work? is it possible?
- [ ] can this work on embedd? with a serial comm TUI? probably need to reseach crossterm



# important note here!
figure out at least something uselful with this because the whole idea is kind of lame and useless..
wtf am I doing with my life..
