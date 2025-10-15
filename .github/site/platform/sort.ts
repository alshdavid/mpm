import semver from "semver";

export function sortEntries(a: string, b: string) {
  try {
    const semverA = tryParseSemver(a);
    const semverB = tryParseSemver(b);

    if (semverA && semverB) {
      return semver.compare(semverB, semverA);
    } else if (semverA) {
      return -1;
    } else if (semverB) {
      return 1;
    } else {
      return b.localeCompare(a);
    }
  } catch (e) {
    return b.localeCompare(a);
  }
}

export function tryParseSemver(str: string): semver.SemVer {
  let result = semver.parse(str)
  if (!result) throw new Error()
  return result
}
