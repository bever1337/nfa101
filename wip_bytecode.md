## requirements

- machines have a q0 state (start) and an f state (end)
- machines have a positive number of states
- every state has one transition label
- every state goes left and right. Nodes are ordered meaning left only or left and right.
- state ids are positive

## preamble

- HEADERS (2 bytes)
  - QID: bytes per qId (1 byte)
  - LABEL: bytes per label (1 byte)
- DECLARATION (QID + QID)
  - q0 (QID)
  - f (QID)
- DELTA
  - goto instruction:
    - transition label (LABEL)
    - left state (QID)
    - right state (QID)

### Bytecode of the RegEx `'a'`

```bash
0000 0001 # each qId is 8 bits
0010 0000 # each transition is 32 bits
0000 0001 # q0 (first state) is 1
0000 0010 # f (final state) is 2
# qId 1
  0000 0000 0000 0000 0000 0000 0100 0001 # 'a'
  0000 0010 # goto 2
  0000 0000 # no right
# qId 2
  0000 0000 0000 0000 0000 0000 0000 0000 # no transition
  0000 0000 # no left
  0000 0000 # no right
```
