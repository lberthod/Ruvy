const functions = require('firebase-functions');
const admin = require('firebase-admin');

admin.initializeApp();

exports.incrementCounter = functions.https.onCall(async (data, context) => {
  try {
    const db = admin.database();
    const counterRef = db.ref('counter');

    const snapshot = await counterRef.once('value');
    const currentValue = snapshot.val() || 0;
    const newValue = currentValue + 1;

    await counterRef.set(newValue);

    return {
      success: true,
      value: newValue,
      message: 'Counter incremented successfully'
    };
  } catch (error) {
    console.error('Error:', error);
    throw new functions.https.HttpsError('internal', error.message);
  }
});

exports.resetCounter = functions.https.onCall(async (data, context) => {
  try {
    const db = admin.database();
    const counterRef = db.ref('counter');

    await counterRef.set(0);

    return {
      success: true,
      value: 0,
      message: 'Counter reset successfully'
    };
  } catch (error) {
    console.error('Error:', error);
    throw new functions.https.HttpsError('internal', error.message);
  }
});

exports.getCounter = functions.https.onCall(async (data, context) => {
  try {
    const db = admin.database();
    const counterRef = db.ref('counter');

    const snapshot = await counterRef.once('value');
    const value = snapshot.val() || 0;

    return {
      success: true,
      value: value
    };
  } catch (error) {
    console.error('Error:', error);
    throw new functions.https.HttpsError('internal', error.message);
  }
});