After reading some interesting articles regarding how effective measures against pandemics work I decided to write my own small simulation app to play with. [This Washington Post article](https://www.washingtonpost.com/graphics/2020/world/corona-simulator/) is mainly what inspired this project. I centered this simulation around shelter in place measures as implemented in the article. In this app, "shelter in place" simply means to stay put.

To keep things simple, infection chances is 100% if an infected organism directly interacts with one that is not infected. In addition, organisms are fully immune to the virus once they have recovered or if they die(of course).

## Tech

This app is a recreational project and so I didn't spend too much time making the code robust or "production ready". It's written in rust using the [rgx graphics library](https://github.com/cloudhead/rgx). RGX was chosen to get up and running as quickly as possible.

## Playing with the numbers

If you choose to run this application yourself and play with the numbers, you can mess with the values used to randomly generate organisms and simulate speed, shelter-in-place percentages, fatality rates, and organism numbers and sizes.

I tried to keep all hard-coded values in their own place in the code(there may have been some that were missed). [You can find them here](https://github.com/0x0caf/pandemic_simulation/blob/master/src/simulation_app.rs#L20) at the initialization phase of the `SimulationApp` object.

```rust
let num_organisms = 6000;
let num_initially_infected = 200;
let percent_in_place = 50.;
let circle_radius = 3.0;
let infection_lifetime_ms = 1000;
let fatality_rate = 2.0;
// proportions
let grid_pixel_size = 25;
let max_velocity = 100.;
```

`num_organisms` - The number of organisms this simulation will create. I can get this app to run up to 20,000 organisms at once without any noticeable effects on FPS. I was seeing some significant slow-downs on FPS with large numbers of newly infected organisms at one time, but that has mainly been resolved with the implementation of the [Grid System](https://github.com/0x0caf/pandemic_simulation/blob/master/src/grid_system.rs).

`num_initially_infected` - This is the number of infected organisms the app will start the simulation with.

`percent_in_place` - the percentage of organisms to shelter in place.

`organism_size` - A variable to set the size of the organisms in pixels.

`infection_lifetime_ms` - the number of milliseconds an organism is to stay infected and contagious with the virus. Afterwards the organism will either recover or die based on `fatality_rate`.

`grid_pixel_size` - The size, in pixels, the Grid System will use to divide the screen into areas in which infected organisms will be recorded. This is to make the newly infected rate not take a hit on FPS. Play with this number only if you wish to see the Grid Systems effect on CPU and rendering times.

`max_velocity` - The max velocity of each organism should they not be sheltering in place. This value is in pixels per second.

## Organism Colors
I've chosen some colors to indicate an organism's state. Feel free to change these colors as you see fit.

**Uninfected** - Green

**Infected** - Red

**Recovered** - Dark Gray

**Dead** - Blinking Fuchsia

## Minimal Usage
There is currently only one way to control this app: Pause and unpause:

* Press Space to unpause. This app starts paused initially
* Press P to pause

## Initial Observations

As stated in the Washington Post article, shelter in place has a noticeable effect on infection rate even with a simple app like this one. The higher the shelter in place percentage the slower the infection and the higher number of uninfected when the virus finally dies out. With lower shelter in place percentages there are a higher number of infected organisms, a quicker infection rate, and a higher number of dead.

Of course this app is far to simple to simulate more realistic characteristics of a pandemic, but it's very interesting to observe how simple characteristics such as shelter in place can have a positive effect.

Ninety Percent Shelter In Place:

![Ninety Percent Shelter In Place](images/Ninty-Percent_small.gif)

Fifty Percent Shelter In Place:

![Fifty Percent Shelter In Place](images/Fifty-Percent_small.gif)

Zero Percent Shelter In Place: 

![Zero Percent Shelter In Place](images/Zero-Percent_small.gif)

## Current Issues
Due to the casualness of my approach to development of this app, I've left some things out of this app.

For one, there is no text for this app. This is mainly centered around RGX's(understandable) lack of a proper text rendering system. I thought about writing my own to make up for this but found it to be out of scope for this project.

Second, there's no way to reset the simulation without restarting the app. This is something I'll probably work on at some point in the future. That way I can get proper screen captures and post gifs/screenshots here.
