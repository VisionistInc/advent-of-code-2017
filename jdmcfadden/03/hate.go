package main

import (
	"math"

	"gonum.org/v1/gonum/mat"
)

// impl from http://oeis.org/A141481
func hateSpiral(size int) []int {

	// m=5;
	m := size

	// h=2*m-1;
	h := 2*m - 1

	// A=matrix(h, h);
	A := mat.NewDense(h, h, make([]float64, h*h, h*h))

	// print1(A[m, m]=1, ", ");
	A.Set(m-1, m-1, 1)

	// T=[[1, 0], [1, -1], [0, -1], [ -1, -1], [ -1, 0], [ -1, 1], [0, 1], [1, 1]];
	T := [...]complex128{
		complex(1, 0),
		complex(1, -1),
		complex(0, -1),
		complex(-1, -1),
		complex(-1, 0),
		complex(-1, 1),
		complex(0, 1),
		complex(1, 1)}

	// for(n=1, (h-2)^2-1,
	result := make([]int, (h-2)*(h-2), (h-2)*(h-2))
	resultIdx := 1
	result[0] = 1 // side-effect from the print statement above!
	for n := 1; n <= (h-2)*(h-2)-1; n++ {
		// 	g=sqrtint(n);
		g := int(math.Floor(math.Sqrt(float64(n))))

		// 	r=(g+g%2)\2;
		r := (g + (g % 2)) / 2

		// 	q=4*r^2;
		q := 4 * r * r

		// 	d=n-q;
		d := n - q

		// 	if(n<=q-2*r,
		var j, k int
		if n <= q-2*r {
			// j=d+3*r;
			j = d + 3*r
			// k=r,
			k = r
		} else {
			// if(n<=q,
			if n <= q {
				// j=r;
				j = r
				// k=-d-r,
				k = -d - r
			} else {
				// if(n<=q+2*r,
				if n <= q+2*r {
					// j=r-d;
					j = r - d
					// k=-r,
					k = -r
				} else {
					// j=-r;
					j = -r
					// k=d-3*r
					k = d - 3*r
				}
			}
		}
		// 	)));

		// 	j=j+m;
		j = j + m

		// 	k=k+m;
		k = k + m

		// 	s=0;
		var s int

		// 	for(c=1, 8,
		for c := 1; c <= 8; c++ {
			// v=[j, k];
			v := complex(float64(j), float64(k))
			// v+=T[c];
			v += T[c-1]
			// s=s+A[v[1], v[2]]
			atI := int(real(v)) - 1
			atJ := int(imag(v)) - 1
			s += int(A.At(atI, atJ))
		}
		// 	);

		// 	A[j, k]=s;
		A.Set(j-1, k-1, float64(s))

		// 	print1(s, ", ")
		result[resultIdx] = s
		resultIdx++
	}
	// )

	return result
}
