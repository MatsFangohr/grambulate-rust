# grambulate-rust

An implementation of grambulation in Rust, as initially explained [on Reddit](https://www.reddit.com/r/mathmemes/comments/tvn2gj/the_solution_to_the_april_fools_math/) and more precisely defined for [Code Golf](https://codegolf.stackexchange.com/questions/259698/implement-grambulation).

## Implementation

If the MathJax doesn't render, try [viewing the README on GitHub](https://github.com/MatsFangohr/grambulate-rust).

To grambulate two numbers $a$ and $b$ to a solution $c$, we must find the coordinates $c'$ of the solution, for which we must first determine the coordinates of $a$ and $b$ ($a'$ and $b'$) on the spiral.

To do so, we can calculate the 'rings' $r_a$ and $r_b$ the numbers are in with the formula
$$r(x)=\left\lfloor{\frac{\left\lceil{\sqrt{x}}\right\rceil}{2}}\right\rfloor$$
Then, the formula
$$v_d(r)=4r^2-(5-d)\cdot{}r+1$$
can be used to to calculate the four values on the four main diagonals for ring $x$, where $d$ specifies which diagonal to use. To do this, we specify the number on a given diagonal in ring 1 as $d$:

| Diagonal     | $d$ |
| ------------ | --- |
| top-right    | 3   |
| top-left     | 5   |
| bottom-left  | 7   |
| bottom-right | 9   |

This gives us 4 values near our desired value (either $a$ or $b$). If we find the value closest to but larger than or equal to our target value, we can calculate the difference and apply it as either an x or y offset, giving us the coordinates of our desired value.

Now that we have both pairs of coordinates, we can calculate the connecting vector $\vec{v}$ and apply to it to $b'$  to find $c'$. 

Once we know $c'$, we know that the ring is the larger of the absolute x and y values.

We now determine which diagonal we need to calculate, retrieve the value using the above formula and subtract the difference of the relevant coordinate.

## Example

Let $a=5$ and $b=11$.

We can use the ring formula to determine $r_a=1$ and $r_b=2$.

Starting with $a$, we can calculate the 4 diagonal values on ring $r_a$:  
$v_3=3$  
$v_5=5$  
$v_7=7$  
$v_9=9$  

The closest of these values that is $\le{}a$ is $v_5=5$. The coordinates of that value are $v_5'=(-1\cdot{}r_a~|+1\cdot{}r_a) = (-1~|~1)$. As $v_5=a$, we don't need any further offsets.

Moving on to $b$:  
$v_3=13$  
$v_5=17$  
$v_7=21$  
$v_9=25$  

The closest of these values that is $\le{}b$ is $v_3=13$. The coordinates of that value are $v_3'=(+1\cdot{}r_b~|+1\cdot{}r_b)=(2~|~2)$. As $v_3\neq{}b$, we still need to do more.

As we determined the closest value _ahead_ of $b$ to be the top-right diagonal, we need to decrease the y coordinate of $v_3'$ by $v_3-b=13-11=2$. That leaves us with:
```math
\vec{c'}=\vec{v_3'}-\begin{pmatrix}0\\2\end{pmatrix}=\begin{pmatrix}2-0\\2-2\end{pmatrix}=\begin{pmatrix}2\\0\end{pmatrix}
```

Now we know:  
```math
a'=(-1~|~1)\hspace{2cm}b'=(2~|~0)
```
The connecting vector is 
```math
\vec{v}=\vec{b'}-\vec{a'}=\begin{pmatrix}2-(-1)\\0-1\end{pmatrix}=\begin{pmatrix}3\\-1\end{pmatrix}
```

Applying this to $b'$ gives us the position vector of $c'$.
```math
$$\vec{c'}=\vec{b'}+\vec{v}=\begin{pmatrix}2+3\\0+(-1)\end{pmatrix}=\begin{pmatrix}5\\-1\end{pmatrix}$$
```
As the value is not directly on a diagonal ($|x|\neq{}|y|$), we can use the following table to determine the diagonal _ahead_ of our target value:

| condition  | diagonal     |
| ---------- | ------------ |
| $\|x\|< y$ | top-left     |
| $x>\|y\|$  | top-right    |
| $x<-\|y\|$ | bottom-left  |
| $-\|x\|>y$ | bottom-right |

We need the top-left diagonal. We also know that our value is on ring $r_c=\max(|5|, |-1|)=5$. The value of the top-left diagonal at ring 5 is, using the formula above, $v_3(5)=91$. 
By definition, $c$ is smaller than that value. As the straight in front of the top-left diagonal is vertical and upwards, we need to subtract the difference between the y-value of 91 and the y-value of $c'$ from 91 and we should find $c$. $$c=91-((y_{91})-(y_{c'}))=91-((+1\cdot{}5)-(-1))=91-6=85$$

That's it! $5~\lozenge{}~11=85$.

## Speed

This approach may seem confusing, but it avoids any loops that change with the inputs; the only loops in my code iterate over the 4 types of `Diagonal`, no more.

This allows very low execution time regardless of input; results from a rough test on my computer:

| `value_a`       | `value_b`        | time per iteration, average of 100'000'000 |
| --------------- | ---------------- | ------------------------------------------ |
| 1               | 2                | 529ns                                      |
| 10              | 25               | 395ns                                      |
| 100000          | 2500000          | 601ns                                      |
| 100000000000000 | 2500000000000000 | 364ns                                      |

These were just the first numbers I chose and this is in no way a proper test, but it'll do for an impression.
