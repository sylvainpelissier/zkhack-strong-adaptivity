# Scheme

Prover computes 

$$\begin{align}
C_1 = \mathrm{COM}(a; r1) = a \cdot G + r_1 \cdot H \\
C_2 = \mathrm{COM}(a; r2) = a \cdot G + r_2 \cdot H
\end{align}$$

Then it sends $C_1,C_2$

Prover computes 

$$\begin{align}
C_\rho = \mathrm{COM}(r; \rho) = r \cdot G + \rho \cdot H \\
C_\tau = \mathrm{COM}(r; \tau) = r \cdot G + \tau \cdot H
\end{align}$$

Then it sends $C_\tau,C_\rho$

In the non interactive version:
$$e = H(pk, C_\tau,C_\rho)$$

Prover computes
$$\begin{align}
    s = r + e  a \\
    u = ρ + e  r_1 \\
    t = τ + e  r_2 \\
\end{align}$$
Then it sends $s, u, t$
# Observation

The goal is to have:


The message itself is not taken into account in the Fiat-Shamir transformation.

We can use another $\tilde{r}$ in the computation of $C_\rho$