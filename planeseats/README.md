<h3>The Problem</h3>
```
There are 100 people in line to board a plane with 100 seats. The first person has lost her boarding pass, so she takes a random seat.
Everyone that follows takes their assigned seat if it's available, but otherwise takes a random unoccupied seat. What is the probability the last passenger ends up in her assigned seat?
```
<h3>The Solution</h3>
```
The first person may select any seat (including their correct one). Therefore, assigning the seats does not matter and each person can be defined as their seat number. I.e. the first person == seat 0.<br>
```
```
Key:
true = last person received their seat
false = last person did not get their seat
```
* Set random # 0-99 array element to true<br>
* Loop through array from 2-98
* If last element is true at any point, the last person's seat was taken -> so fail
* If loop completes, return true: last person's seat was left over<br>
  It is not actually necessary to set the last element to true if we have reached the last position
