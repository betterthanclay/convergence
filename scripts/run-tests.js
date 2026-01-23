const { spawnSync } = require('child_process');

function run(cmd, args, opts) {
  const res = spawnSync(cmd, args, { stdio: 'inherit', ...opts });
  if (typeof res.status === 'number') process.exit(res.status);
  process.exit(1);
}

function hasNextest() {
  const res = spawnSync('cargo', ['nextest', '--version'], { stdio: 'ignore' });
  return typeof res.status === 'number' && res.status === 0;
}

const args = process.argv.slice(2);

if (process.env.FORCE_CARGO_TEST === '1') {
  run('cargo', ['test', ...args]);
}

if (hasNextest()) {
  run('cargo', ['nextest', 'run', ...args]);
} else {
  run('cargo', ['test', ...args]);
}
