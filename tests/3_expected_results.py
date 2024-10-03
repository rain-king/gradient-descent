import sympy as sp

# Define the variables
x1, x2, x3, x4 = sp.symbols('x1 x2 x3 x4')

# Define the function
f = (x1 - 2)**2 + (x2 + 1)**2 + (x3 - 3)**2 + (x4 + 4)**2 + x1*x2 - x3*x4

# Compute the gradient of the function
grad_f = [sp.diff(f, var) for var in (x1, x2, x3, x4)]

# Solve the system of equations grad_f = 0
solution = sp.solve(grad_f, (x1, x2, x3, x4))

# Evaluate the function at the solution point(s)
optimal_value = f.subs(solution)

solution, optimal_value

# ({x1: 10/3, x2: -8/3, x3: 4/3, x4: -10/3}, 10/3)
