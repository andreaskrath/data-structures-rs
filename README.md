# Branching
## Naming
Branches should be named so that they can lead back to the issue they relate to, and so as to indicate the purpose of the branch.
This is done by using the following template `<issue-type>/<issue-number>`, for example:
- `feature/225`
- `bug/102`
- `documentation/155`

## Merging
Merges to main should only happen once all actions are completed and the issue is solved in its entirety.

# Safety
The entirety of the crate is written in safe rust, enforced with `#![forbid(unsafe_code)]`.

# Convention
The product should be developed with the [API guidelines](https://rust-lang.github.io/api-guidelines/about.html) in mind.
