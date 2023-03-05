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
        rules.append(rule)
        buffer.clear()

root = CombinatorialClass.from_dict(json.loads(buffer[0]))

spec = CombinatorialSpecification(root, rules, group_equiv=False)
# spec.sanity_check(10)
# spec.show(verbose=True)
for n in range(10):
    brute_force = sum(1 for _ in root.objects_of_size(n))
    spec_count = spec.count_objects_of_size(n)
    print(n, brute_force, spec_count)
    if spec_count != brute_force:
        for t in spec.comb_classes():
            print(spec.get_label(t), spec.get_rule(t).count_objects_of_size(n))
        break
