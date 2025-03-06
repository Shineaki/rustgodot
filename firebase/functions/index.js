/**
 * Import function triggers from their respective submodules:
 *
 * const {onCall} = require("firebase-functions/v2/https");
 * const {onDocumentWritten} = require("firebase-functions/v2/firestore");
 *
 * See a full list of supported triggers at https://firebase.google.com/docs/functions
 */

const functions = require('firebase-functions/v1');
// const auth = require('firebase-functions/v2');
const logger = require("firebase-functions/logger");
const admin = require('firebase-admin');
const { v4: uuidv4 } = require('uuid');
admin.initializeApp();
const firestore = admin.firestore();

exports.create_db_entry = functions.region("europe-west6").auth.user().onCreate((user) => {
    firestore.collection('characters').doc(user.uid).set({
        "CharacterName1": {
            id: uuidv4(),
            name: "CharacterName1",
            level: 1,
            type: 1
        },
        "CharacterName2": {
            id: uuidv4(),
            name: "CharacterName2",
            level: 10,
            type: 2
        }
    });
});