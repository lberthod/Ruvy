const functions = require('firebase-functions');
const admin = require('firebase-admin');
const cors = require('cors')({ origin: true });

admin.initializeApp();

exports.incrementCounter = functions.https.onRequest((req, res) => {
  cors(req, res, async () => {
    try {
      const db = admin.database();
      const counterRef = db.ref('counter');

      const snapshot = await counterRef.once('value');
      const currentValue = snapshot.val() || 0;
      const newValue = currentValue + 1;

      await counterRef.set(newValue);

      res.json({
        success: true,
        value: newValue,
        message: 'Counter incremented successfully'
      });
    } catch (error) {
      console.error('Error:', error);
      res.status(500).json({
        success: false,
        error: error.message
      });
    }
  });
});

exports.resetCounter = functions.https.onRequest((req, res) => {
  cors(req, res, async () => {
    try {
      const db = admin.database();
      const counterRef = db.ref('counter');

      await counterRef.set(0);

      res.json({
        success: true,
        value: 0,
        message: 'Counter reset successfully'
      });
    } catch (error) {
      console.error('Error:', error);
      res.status(500).json({
        success: false,
        error: error.message
      });
    }
  });
});

exports.getCounter = functions.https.onRequest((req, res) => {
  cors(req, res, async () => {
    try {
      const db = admin.database();
      const counterRef = db.ref('counter');

      const snapshot = await counterRef.once('value');
      const value = snapshot.val() || 0;

      res.json({
        success: true,
        value: value
      });
    } catch (error) {
      console.error('Error:', error);
      res.status(500).json({
        success: false,
        error: error.message
      });
    }
  });
});