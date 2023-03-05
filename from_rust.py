from comb_spec_searcher import CombinatorialClass, Strategy, CombinatorialSpecification
import json

import sys

print("=" *80)
print("              Welcom to python")
print("=" *80)

rules = []
buffer: list[str] = []
for line in sys.stdin:
    buffer.append(line.rstrip())
    if len(buffer) == 2:
        comb_class = CombinatorialClass.from_dict(json.loads(buffer[0]))
        strat = Strategy.from_dict(json.loads(buffer[1]))
        rule = strat(comb_class)
        if rule.is_equivalence():
            rule = rule.to_equivalence_rule()
        rules.append(rule)
        buffer.clear()

root = CombinatorialClass.from_dict(json.loads(buffer[0]))

spec = CombinatorialSpecification(root, rules, group_equiv=False)
spec.sanity_check(5)
spec.show(verbose=True)
for n in range(10):
    print(spec.count_objects_of_size(n))
spec._group_equiv_in_path()
