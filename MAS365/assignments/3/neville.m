% Neville's Method

% Define f(x)
f = @(x) 3^x;          % for (a)
% f = @(x) sqrt(x);    % for (b)

X = [-2, -1, 0, 1, 2];
N = length(X);

% Q{i, j} = Q_{i - 1, j - 1}
Q = cell(N, N);

for i = 1:N
    Q{i, 1} = [f(X(i))];
end

% Calculating Q{i, j}
for j = 2:N
    for i = j:N
        % (x - x_{i-j})Q_{i, j-1}
        first_term = sum_poly(    ...
            [Q{i, j-1} 0],        ...
            -X(i-j+1) * Q{i, j-1} ...
        );
        % -(x - x_{i})Q_{i-1, j-1}
        second_term = sum_poly(   ...
            -1 * [Q{i-1, j-1} 0], ...
            X(i) * Q{i-1, j-1}    ...
        );
        
        Q{i, j} =                               ...
            sum_poly(first_term, second_term) / ...
            (X(i) - X(i-j+1));
    end
end

% Evaluate the polynomial
x = 0.5;    % for (a)
% x = 3;    % for (b)
fprintf('f(%f) = %.8f\n', x, polyval(Q{N, N}, x))

% x * P = [P 0]
% a * P = a * P
% P + Q
function s = sum_poly(a, b)
    N = max(length(a), length(b));
    pa = [zeros(1, N - length(a)) a];
    pb = [zeros(1, N - length(b)) b];
    s = pa + pb;
    return
end
