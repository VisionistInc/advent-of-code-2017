Part 2 is a mess, but you can probably extract an answer from it. 
In my case, it prints out:

```
One of these is not like the other: jnyexah: 1786, ltleg: 1792, fdnmqri: 1786, iysgr: 1786, dffdie: 1786, vvgyhb: 1786
jnyexah (834) -> dpqxwea (162, 238 (total)), ckguaj (238, 238 (total)), eqjoky (166, 238 (total)), asxxcu (13, 238 (total))
ltleg (808) -> vnlmjy (146, 164 (total)), bscjk (66, 164 (total)), uycjw (66, 164 (total)), kzuimi (110, 164 (total)), kwlqal (104, 164 (total)), lahieha (20, 164 (total))
fdnmqri (29) -> afucrtw (119, 251 (total)), gpium (109, 251 (total)), pcissqn (229, 251 (total)), hgayc (155, 251 (total)), satrfh (133, 251 (total)), rlsynsa (63, 251 (total)), yutvqn (225, 251 (total))
iysgr (586) -> lucoli (226, 400 (total)), qnioy (400, 400 (total)), ybhtg (56, 400 (total))
dffdie (786) -> nnwxxk (64, 250 (total)), aeppvjo (74, 250 (total)), wdzqhs (102, 250 (total)), hogtrz (214, 250 (total))
vvgyhb (526) -> ruqgy (306, 420 (total)), xrgsnsh (24, 420 (total)), wytylnc (398, 420 (total))
```

so you look at which name is different than the others, and then you look at the difference between it and its neighbors, and then you subtract that from the weight of itself. 

In this instance `ltleg` (which I've been pronouncing "leet-leg" because it sounds cool) is the problem, its total weight is 1792, but all of its neighbors have weight 1786, for a difference of 6. 

So take `ltleg`'s weight and subtract 6, that's my answer.  (in this case)
