triangle(A, A, A, "equilateral") :- A > 0.

triangle(A, B, C, "isosceles") :- 
    valid(A, B, C),
    isosceles(A, B, C).

triangle(A, B, C, "scalene") :- 
    valid(A, B, C), 
    \+ isosceles(A, B, C), 
    \+ equilateral(A, B, C).

isosceles(A, A, _).
isosceles(A, _, A).
isosceles(_, A, A).

equilateral(A, A, A).

valid(A, B, C) :- 
    A > 0, 
    B > 0, 
    C > 0, 
    A + B >= C,
    B + C >= A, 
    A + C >= B.
    