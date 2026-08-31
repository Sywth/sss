# Logic 

This is the shared language between all the decision problems we implement

## High Level Definitions 

|Type|Definition |
|---|-------------------|
|Term| Something that denote an object in the domain.|
| Symbol| An identifier. In FOL it can refer to syntactic character of FOL itself (e.g. $\lor$) or a character introduced as part of a specific sentence (e.g. a constant, variable or predicate).|
| Formula| Any application of a predicate (including equality), logical operator ($\lor$, $\neg$, ...), quantifier ($\forall$ or $\exists$). Always evaluates to $\top$ or $\bot$ over sufficent interperation.|

## First Order Logic 

First order logic (FOL) is defined via the 5-tuple 


$$
\text{FOL} \coloneqq (
    \Sigma,
    \text{Vars},
    \text{Term}_{\Sigma}(\text{Vars}),
    \text{Form}_{\Sigma}(\text{Vars}),
    \vDash
    )
$$ 

where $\Sigma$ is the 4-tuple defining some specific FOL signature 

$$
\Sigma \coloneqq
    \left(
    \text{Consts},
    \text{Fns},
    \text{Rels},
    \text{arities}
    \right)
$$
