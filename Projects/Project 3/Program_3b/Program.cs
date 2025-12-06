using System;
using System.Collections.Generic;
using System.Text.RegularExpressions;

namespace FlightCheck;

partial class Program
{
    /// <summary>
    /// graph that holds the routes between cities and uses case insensitive keys
    /// </summary>
    private static readonly Dictionary<string, HashSet<string>> _routes = new(StringComparer.OrdinalIgnoreCase);


    /// <summary>
    /// regex for parsing flight(city1,city2)
    /// </summary> 
    [GeneratedRegex("^flight\\(\\s*([A-Za-z ]+)\\s*,\\s*([A-Za-z ]+)\\s*\\)$")]
    private static partial Regex Flight_Regex();

    static void Main(string[] args)
    {

        if (args.Length == 0)
        {
            Console.WriteLine("Usage: Program.exe \"flight(start, end)\"");
            return;
        }


        CreateGraph();


        Match content = Flight_Regex().Match(args[0]);

        string city1 = content.Groups[1].Value.Trim();
        string city2 = content.Groups[2].Value.Trim();

        // 3. Execute Imperative Search
        bool result = HasRoute(city1, city2);

        // 4. Output result
        Console.WriteLine($"{result}.");

    }

    /// <summary>
    /// sets the default routes for program
    /// </summary>
    static void CreateGraph()
    {


        AddRoute("fresno", "seattle");
        AddRoute("fresno", "albany");
        AddRoute("fresno", "boston");

        AddRoute("seattle", "omaha");


        AddRoute("omaha", "albany");

        AddRoute("albany", "dallas");

        AddRoute("dallas", "seattle");
        AddRoute("dallas", "albany");

        AddRoute("atlanta", "dallas");
        AddRoute("atlanta", "albany");
    }

    static void AddRoute(string from, string to)
    {
        if (!_routes.ContainsKey(from)) _routes[from] = [];

        _routes[from].Add(to);
    }

    /// <param name="start"></param>
    /// <param name="end"></param>
    /// <returns><see langword="true"/> if you can eventually get to the <paramref name="end"/> city from the given <paramref name="start"/> city</returns>
    static bool HasRoute(string start, string end)
    {

        // end if start doesnt exist
        if (!_routes.ContainsKey(start)) return false;

        // Queue for BFS frontier
        Queue<string> queue = new([start]);


        // Set for visited nodes to prevent infinite loops (cycles)
        HashSet<string> visited = [start];


        string current;
        while (queue.Count != 0)
        {
            current = queue.Dequeue();

            // end if destination found 
            if (current.Equals(end, StringComparison.OrdinalIgnoreCase)) return true;

            // get the neighbors 
            if (_routes.TryGetValue(current, out HashSet<string>? value))
                foreach (string neighbor in value)
                    if (visited.Add(neighbor)) queue.Enqueue(neighbor);

        }

        return false;
    }


}
