## Prerequisites

-   Project was created in Windows 11
-   **SWI-Prolog or the browser version** for Programs 1, 3a, 4, 5, 6) (skipped program 2)
-   I used .NET 10 for Program 3b but I think it is probably backwards compatible
-   The commands below work for if the **current directory** is the same one this file and the word doc are in

## Program 1

**Folder:** `Program 1/prog1.pl`

**Command line**

```bash
swipl "Program 1\prog1.pl"
```

**swipl**

```prolog
translate(Chinese, [one, two]).
translate([ling, yi, er], English).
```

## Program 3a:

**Folder:** `Program 3/prog3.pl`

**Command Line:**

```bash
swipl "Program 3\prog3.pl"
```

**swipl**

```prolog
flight(fresno, seattle).
flight(seattle, atlanta).
flight(dallas, boston).
```

## Program 3b

**Folder:** `Program 3/Program_3b/`

**Command Line**

```bash
cd "Program 3\Program_3b" && dotnet run
```

same as prolog but no dot at the end

```bash
dotnet run "flight(fresno, seattle)"
dotnet run "flight(seattle, atlanta)"
```

## Program 4

**Folder:** `Program 4/prog4.pl`

**Command Line**

```bash
swipl "Program 4\prog4.pl"
```

**swipl**

```prolog
is_teaching(dehlinger, time(t, 1100)),
is_instructor(mallik, cosc465),
is_busy(sai, w, 1900, location(yr7800, 125)),
cannot_meet(time(t, 1100), dehlinger, mallik),
schedule_conflict(cosc455, cosc236).
```

## Program 5

**Folder:** `Program 5/prog5.pl`

**How to Run:**

```bash
swipl "Program 5\prog5.pl"
```

**swipl**

```prolog
suspect(Person).
motive(Person, greed).
motive(Person, hatred).
spouse(X, Y).
```

## Program 6

**Folder:** `Program 6/prog6.pl`

**Command Line**

```bash
swipl "Program 6\prog6.pl"
```

**swipl**

```prolog
prog6(HomerPath).
```
