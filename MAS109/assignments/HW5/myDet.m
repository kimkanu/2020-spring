function d = myDet(A)
    n = sizeOfSquare(A);
    if n == -1
        e = MException('The input is not square!');
        throw(e)
    elseif n == 0
        d = 1;
    elseif n == 1
        d = A;
    else
        d = sum(arrayfun(                                                       ...
            @(i) (-1)^(i + 1) * A(i, 1) * myDet(removeRowCol(A, i, 1)), ...
            1:n                                                         ...
        ));
    end
    return
end

function b = sizeOfSquare(A)
    s = size(A);
    if length(s) ~= 2
        b = -1;
    elseif s(1) ~= s(2)
        b = -1;
    else
        b = s(1);
    end
    return
end

function B = removeRowCol(A, i, j)
    [m, n] = size(A);
    B = [A(1:i-1, 1:j-1), A(1:i-1, j+1:n);
         A(i+1:m, 1:j-1), A(i+1:m, j+1:n)];
    return 
end
