# TODO
- [ ] Write basic S-expression BNF for ZOL and FOL 
- [ ] Write basic S-expression parser for ZOL and FOL
- [ ] Write ZOL and FOL normalization algorithms
    - [ ] Given a ZOL file spit out the CNF ZOL file 
        - [ ] The minimal variable version should be at best exp time 
        - [ ] Use testin transform which introduces variables making it P time 
    - [ ] Given a FOL file spit out a prenex FOL file 
    - [ ] Tarnsform FOL further into Skolem Normal form 


- [ ] Refactor the whole code base so the s-expressions are default input 


- [ ] Think about how we will structure these transform and decision problem 
    algorithms in our code base 
- [ ] Write a FOL solver in its own crate? 
- [ ] Write a QBF solver in its own crate? 
- [ ] Put the SAT solver in its own crate? 
- [ ] Re-write / optimize the SAT solver
- [ ] Re-write the parsers for DIMACS and QDIMACS


/*
 * Simple usuage 
 * $ sss sat --dimacs sat_example.dimacs
 * $ sat 
 *
 * $ sss sat --qdimacs unsat_example.qdimacs
 * $ unsat
 *
 * $ sss sat unsat_example.sfol
 * $ unsat
 *
 * $ sss sat loop_example.sfol
 * $ timeout 
 * 
 * $ sss sat --max-time=1h20m long_example.sfol
 * $ sat 
 *
 * $ sss cast file_name.dimacs 
 * // creates file called file_name.sfol
 *
 * $ sss norm file_name.sfol
 * // creates file called file_name.normalized.sfol
 */
