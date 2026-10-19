#!/usr/bin/env python3
import json,subprocess,sys
m=json.loads(subprocess.check_output(['cargo','metadata','--locked','--no-deps'],text=True))
packages={p['id']:p for p in m['packages']}; names={p['name'] for p in m['packages']}; reverse={n:0 for n in names}
for p in m['packages']:
    for d in p['dependencies']:
        if d['name'] in reverse: reverse[d['name']]+=1
roots={'cf-cli'}
orphans=[n for n,c in reverse.items() if c==0 and n not in roots]
print(json.dumps({'packages':len(packages),'internal_edges':sum(reverse.values()),'orphans':orphans},indent=2))
if len(packages)<20 or orphans: sys.exit(1)
