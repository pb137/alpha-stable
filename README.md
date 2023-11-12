# alpha-stable
Sample and generate probability distribution functions of Alpha-Stable distributions in Rust.

## Specifying the distribution
Distributions are specified using one of two forms:
 - Standard form: S_alpha(sigma, beta, mu) - equivalent to the 'first parameterization' of [Wikipedia](<https://en.wikipedia.org/wiki/Stable_distribution>) with c = sigma.
 - Nolan's form: S^0_alpha(sigma, beta, mu_0) - equivalent to the 'second parameterization' in Wikipedia with delta = mu_0 and gamma = sigma.

## Examples: random walks
Here are three random walks generated by examples/distribution/main.rs

### Alpha = 2 - a normal distribution
![Normal random walk](https://raw.githubusercontent.com/pb137/alpha-stable/main/images/Samples_3.png?raw=true)

### Alpha = 1.5 
![Alpha = 1.5](https://raw.githubusercontent.com/pb137/alpha-stable/main/images/Samples_2.png?raw=true)

### Alpha = 1.1 
![Alpha = 1.1](https://raw.githubusercontent.com/pb137/alpha-stable/main/images/Samples_1.png?raw=true)

## Examples: distributions
A set of probability distribution functions specified in Standard and Nolan's forms.

### Standard form
![Standard form](https://raw.githubusercontent.com/pb137/alpha-stable/main/images/Distributions_1.png?raw=true)

### Nolan's form
![Nolan's form](https://raw.githubusercontent.com/pb137/alpha-stable/main/images/Distributions_2.png?raw=true)

## Examples: sample histograms
Some sample histograms to show samples and probability distribtions are consistent.

### Alpha = 1.1
![Alpha = 1.1](https://raw.githubusercontent.com/pb137/alpha-stable/main/images/Hist_1.png?raw=true)

### Alpha = 1.5
![Alpha = 1.5](https://raw.githubusercontent.com/pb137/alpha-stable/main/images/Hist_2.png?raw=true)

### Alpha = 2 - a normal distribution
![Alpha = 2.0](https://raw.githubusercontent.com/pb137/alpha-stable/main/images/Hist_3.png?raw=true)

### Alpha = 1.1; Beta = 0.5 
![Alpha = 1.1; Beta = 0.5](https://raw.githubusercontent.com/pb137/alpha-stable/main/images/Hist_4.png?raw=true)
