function [a, b]=LS_solver_cs(x, y, opt)
% --------- function file "LS_solver.m" --------- %
% input data: x, y and opt
%   if opt=1, linear model (y=a*x+b)
%   if opt=2, exponential model (y=a*exp(b*x))
%   if opt=3, logarithmic model (y=a+b*ln(x))

[m1, n1]=size(x);   [m2, n2]=size(y);   % size of input data
xx=linspace(min(x), max(x), 100);   % xx for plotting the fitting curve.
if (m1~=1)||(m2~=1)||(n1~=n2)       % If the input data size is not proper
    fprintf('Error: Improper input data.\n');   % error message.
elseif (opt==1)||(opt==2)||(opt==3) % option = 1, 2, 3
    figure; plot(x, y, 'o');        % Plotting the given data points
    hold on;                        % Ready to draw the next graph
    switch opt
        case 1 % Linear model
            fprintf('Linear model\n');

            % ---- BLANK 1 ---- %
            sol = [x', ones(n1, 1)] \ y';

            a=sol(1); b=sol(2);     % fitting constant a and b
            plot(xx, a*xx+b);       % Plot the fitting curve with obtain a and b
            title('Linear model (y=a*x+b)');
            
        case 2 % Exponential model
            fprintf('Exponential model\n');

            % ---- BLANK 2 ---- %
            signs = arrayfun(@(t) sign(t), y);
            if range(signs) ~= 0
                fprintf('Error: Improper input data.\n');
                return;
            end

            sgn = signs(1);
            sol = [ones(n1, 1), x'] \ log(sgn * y');
            a = sgn * exp(sol(1));
            b = sol(2);
            plot(xx, a*exp(b*xx));       % Plot the fitting curve with obtain a and b
            title('Exponential model (y=a*exp(b*x))');
        
        case 3 % Logarithmic model
            fprintf('Logarithmic models\n');

            % ---- BLANK 3 ---- %
            signs = arrayfun(@(t) sign(t), x);
            if (range(signs) ~= 0) || (signs(1) ~= 1)
                fprintf('Error: Improper input data.\n');
                return;
            end

            sol = [ones(n1, 1), log(x')] \ y';
            a = sol(1);
            b = sol(2);
            plot(xx, a + b * log(xx));       % Plot the fitting curve with obtain a and b
            title('Logarithmic model (y=a+b*ln(x))');
        
    end
    hold off;       % no more graph
else                % for invalid [opt]
    fprintf('Error: Improper option value.\n'); % error message
    return;         % Return the process.
end
end