# Final-Project-210

WRITEUP

My project does a number of interesting things, and works with any directed graph provided it is not too large for the code to handle using i64s. The code takes a directed graph, reads it and writes it so it is useable, and then determines a number of different interesting things about the graph. These are: Average Degree of Separation, Largest Degree of Separation, Clustering Coefficient, Number of Nodes, Number of Edges, Node with Highest Degree, and Node with Lowest Degree. The particular graph which I am using comes from SNAP, and is a graph showing all emails sent and received by a number of people from a European research institution. Interestingly, the highest degree of separation was only 7, which is relatively long considering the idea of "six degrees of separation" but still smaller than one would intuitively expect. The average degree of separation was 2.65, which just goes to show the interconnectedness of research and how quick it is to traverse a social network in general as a rule. The graph also has a .3905 clustering coefficient.

The code can be ran via cargo run, and then typing in the file which is wanted to be analyzed (i.e. "email-Eu-core.txt"). The code will then run on the selected file and perform a complete analysis. This is done in a fairly timely manner for my graph, however it may prove to take a fairly long time for more complicated graphs.

The output of the function using my data looks like this:

Processing Data...

Average Degree of Separation: 2.65
Largest Degree of Separation: 7
Clustering Coefficient: 0.3905
Number of Nodes: 1005
Number of Edges: 25571
Node with Highest Degree: 160
Node with Lowest Degree: 78

Also included in the files within the project is a test graph used to run tests to validate the code. 


