test_file = open('test.asm', 'r')
output_file = open('output.asm', 'r')

test_file_lines = [line.strip('\n') for line in test_file.readlines()]
output_file_lines = [line.strip('\n') for line in output_file.readlines()]

print(f'Test file lines: {len(test_file_lines)}')
print(f'Output file lines: {len(output_file_lines)}')

for i in range(len(test_file_lines)):
    test_line = test_file_lines[i]
    output_line = output_file_lines[i]

    if test_line.lower() != output_line.lower():
        print(f'TEST: {test_line.lower()}')
        print(f'OUTPUT: {output_line.lower()}')
