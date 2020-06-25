x = 1;
N = 5;

p = zeros(1, 1 + N + 2);
p(1) = 1;

% p_j = p[j+1]
for j=1:N+2
    p(j+1) = p(j) + (-x)^(j) / factorial(j);
end

hp = zeros(1, 1 + N);
for j=1:1+N
    hp(j) = ...
        p(j) - ((p(j+1) - p(j))^2) / ...
        (p(j+2) - 2 * p(j+1) + p(j));
    fprintf('\\hat p_{%d} = %.5f\n', j-1, hp(j))
end

fprintf('true value: %.5f\n', exp(-1))