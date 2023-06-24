from os import system, environ
import pathlib
import shutil
from subprocess import check_output, run

REPO = 'git@github.com:mirkootter/math-demo.git'

short_hash = check_output(['git', 'rev-parse', '--short', 'HEAD']).decode('utf-8').strip()
print("Current repository hash:", short_hash)

commit_message = f"Update to {short_hash}"

check_output(['git', 'clone', '--bare', REPO])

last_commit = check_output(['git', 'log', '-1', '--pretty=format:%s'], cwd='math-demo.git').decode('utf-8')
shutil.rmtree('math-demo.git')

if last_commit == commit_message:
    print("Already up to date")
    exit()

print("Deploy")
run(['git', 'init', '-b', 'main'], cwd='demo/dist', check = True)
run(['git', 'config', 'user.name', 'rustmath-bot'], cwd='demo/dist', check = True)
run(['git', 'config', 'user.email', ''], cwd='demo/dist', check = True)
run(['git', 'add', '-A'], cwd='demo/dist', check = True)
run(['git', 'commit', '-m', commit_message], cwd='demo/dist', check = True)
run(['git', 'remote', 'add', 'origin', REPO], cwd='demo/dist', check = True)
run(['git', 'push', '-f', 'origin', 'main'], cwd='demo/dist', check = True)