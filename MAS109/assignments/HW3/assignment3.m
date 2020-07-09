% 3 (a)
t = linspace(-1, 1, 10000);

R = 3;
r = 1;
eta = pi/3;

p = 6;

phi = 2*pi*t;
theta = 2*pi*p*t;

plot3(                                     ...
    R*cos(phi) + r*(cos(theta).*cos(eta)), ... % M
    R*sin(phi) + r*sin(theta),             ...
    r*(cos(theta).*sin(eta)),              ...                          ...
    R*cos(2*pi*linspace(0,1,100)),         ... % C
    R*sin(2*pi*linspace(0,1,100)),         ...
    zeros(100)                             ...
);