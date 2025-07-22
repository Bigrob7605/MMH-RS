import argparse
import sys
from mmh import fold, unfold, verify


def main():
    parser = argparse.ArgumentParser(description='MMH Storage CLI')
    subparsers = parser.add_subparsers(dest='command')

    # Fold command
    fold_parser = subparsers.add_parser('fold', help='Hash file to 128-bit seed')
    fold_parser.add_argument('file', type=str, help='Input file to hash')

    # Unfold command
    unfold_parser = subparsers.add_parser('unfold', help='Unfold seed to data')
    unfold_parser.add_argument('seed', type=str, help='Hex seed (128 bits)')
    unfold_parser.add_argument('size', type=int, help='Number of bytes to generate')
    unfold_parser.add_argument('--out', type=str, default=None, help='Output file (default: stdout)')

    # Verify command
    verify_parser = subparsers.add_parser('verify', help='Verify file matches seed')
    verify_parser.add_argument('seed', type=str, help='Hex seed (128 bits)')
    verify_parser.add_argument('file', type=str, help='File to verify')

    args = parser.parse_args()

    if args.command == 'fold':
        with open(args.file, 'rb') as f:
            data = f.read()
        seed = fold(data)
        print(seed.hex())

    elif args.command == 'unfold':
        seed = bytes.fromhex(args.seed)
        data = unfold(seed, args.size)
        if args.out:
            with open(args.out, 'wb') as f:
                f.write(data)
        else:
            sys.stdout.buffer.write(data)

    elif args.command == 'verify':
        seed = bytes.fromhex(args.seed)
        with open(args.file, 'rb') as f:
            data = f.read()
        if verify(seed, data):
            print('OK: Data matches seed')
        else:
            print('FAIL: Data does not match seed')
    else:
        parser.print_help()

if __name__ == '__main__':
    main() 