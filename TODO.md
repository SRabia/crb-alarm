CRB alarm 

# first sweep 
- [X] integrate clap with timeout 
- [X] have a notification when timeout
- [X] display time in hms 
- [X] play music with rodio(or else) at timeout


# what's next
after the first inital working skeleton, that I call the first sweep. just having the basic.
These are the details of idea of where it can go next

## async
- [ ]  integrate tokio and make app async and event-based

## the animation
- [X] animation zigzag
- [X] animation archimedean spiral
- [ ] animation change depending on duration(long animation for long timeout)
- [X] overlay progression have a gray full animation to indicate progression
- [ ] for long timeout make appropriate animation, have long vs short animation selection
- [X] select random animation
- [X] select random marker that match the best

## font letter/number
reaseach how font work, how can we import font to display on terminal
- [ ] have numbers symbols large for timeout: update check https://crates.io/crates/tui-big-text
- [X] better timeout displaying

## music player
- [X] how to integrate spotify api or something play with a given time?
- [ ] start music __easy__
- [ ] stop music __easy__
- [ ] increase volume as timeout reaching 0 
- [ ] stop music when timeout increase after reach 0

## general refactor
- [X] separate module into different widget, app, chrono, ...
- [ ] rework the layout ui better seperation of ui.
- [ ] handle event with event better
- [ ] configure the app with a configuration json: all random configurable

## event
- [ ] refator event handle with context

 ## spotify
- [ ] have a connect to spotify "kind of button" to connect: update ui part is done
- [ ] make the info log from rspotify to redirect to a ui
- [ ] the client id and secret need to be injected from client have a prompt with indicatation on how to do it.
- [ ] will have to go full async to simplify auth!
- [ ] figure out how to setup token_cache json and .env client_id an client secret
 
 ## control
- [ ] have a start stop key binding __easy__
- [ ] have a option, arg to play music during time or after time
- [ ] play crb snipped on some control 
- [ ] better increase /decrease timeout that sync better with animation


## the github workflow..
- [ ] workflow with git release a version.. I don't what to do with this mess

## genral improvement and things
- [ ] animation circle sucks with large timeout
- [ ] in general the application is slow less than 30fps(on my shitty laptop though) this is because it debug mode!
- [ ] write the readme and explain 
- [ ] not stable, it crash when increase/reduce time
- [ ] test on different os/platform look like audio is not working smooth on all platform
- [ ] collect all color theme into one module (theme) __easy__
- [ ] Theme module to get all color from eventually configurable

## bug to fix maybe
- [ ] zig zag is glitchy pattern is off the screen

## research
- [X] spotify api how it work? is it possible?
- [ ] can this work on embedd? with a serial comm TUI? probably need to reseach crossterm



