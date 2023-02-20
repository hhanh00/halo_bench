import 'package:flutter/material.dart';
import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const HomePage(),
    );
  }
}

class HomePage extends StatefulWidget {
  const HomePage({Key? key}) : super(key: key);
  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  int _seed = 0;
  bool _running = false;
  String _message = "";

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Halo Test/Benchmark'),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            ElevatedButton(
                onPressed: _onStartStop,
                child: _running ? Text("Stop") : Text("Start")),
            Text("Current Iteration $_seed"),
            Text(_message),
          ],
        ),
      ),
    );
  }

  _onStartStop() async {
    if (_running) {
      _running = false;
    } else {
      _running = true;
      Future(() async {
        while (_running) {
          await test();
          setState(() {});
        }
      });
    }
  }

  Future<void> test() async {
    try {
      print("Test seed: $_seed");
      await api.testFromSeed(seed: _seed);
      _seed += 1;
    }
    catch(e) {
      _running = false;
      _message = e.toString();
      setState(() {});
    }
  }
}
