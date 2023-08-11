# Walk on Spheres
A basic implementation of the Walk on Spheres algorithm (in Rust!), introduced to computer graphics by Sawhney and Crane 2020 (and originally developed by Muller 1956). The algorithm can be used for solving linear partial differential equations using a Monte-Carlo method, with Dirichlet boundary conditions. 

Presently, the implementation is very simple, and can generate outputs like the following:

![Laplace equation solution with Dirichlet boundary conditions (sin with frequency of 2) on a box](outputs/box_2.png)
![Laplace equation solution with Dirichlet boundary conditions (sin with frequency 5) on a box](outputs/box_5.png)
![Laplace equation solution with Dirichlet boundary conditions (sin with frequency 2) on a Circle](outputs/circle_2.png)
![Laplace equation solution with Dirichlet boundary conditions (sin with frequency 5) on a Circle](outputs/circle_5.png)


