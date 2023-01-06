import 'dart:io';

void main(List<String> args) {
  final start = DateTime.now().millisecondsSinceEpoch;

  Directory('files').createSync();

  for (var i = 1; i <= 1000; i++) {
    File('files/test.file.$i.txt')
      ..writeAsStringSync('test file $i')
      ..createSync();
  }

  final end = DateTime.now().millisecondsSinceEpoch;

  print('Time taken: ${end - start} ms');
}
