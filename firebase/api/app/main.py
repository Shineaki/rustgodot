from uuid import uuid4

from app.auth_guard import get_user_token
from app.schemas.character import Character
from fastapi import Depends, FastAPI
from firebase_admin import credentials, firestore, initialize_app
from google.cloud.firestore_v1.document import (DocumentReference,
                                                DocumentSnapshot)

cred = credentials.Certificate("sa.json")
initialize_app(credential=cred)
db = firestore.client()

app = FastAPI()


@app.get("/")
async def root():
    # This is not accessible, because only /api/* is routed to the API
    return {"message": "Hello World"}


@app.get("/api/user")
async def get_current_user(user=Depends(get_user_token)):
    return user


@app.post("/api/create_character")
async def create_character(char: Character, user=Depends(get_user_token)):
    player_id = user["uid"]

    # Get currently stored data
    doc_ref: DocumentReference = db.collection("characters").document(player_id)
    doc: DocumentSnapshot = doc_ref.get()
    current_data = doc.to_dict()

    # to_dict returns None if the document does not exist
    if current_data == None:
        current_data = {}

    # Some validation
    if len(char.name) < 3 or len(char.name) > 20:
        return {"error": "Name must be between 3 and 20 characters"}
    if char.type < 1 or char.type > 4:
        return {"error": "Type must be between 1 and 4"}
    if len(current_data) >= 4:
        return {"error": "You can only have 4 characters"}

    # Update the character data with a new entry
    character_id = uuid4().hex
    char.level = 1
    current_data[character_id] = char.model_dump()

    doc_ref: DocumentReference = db.collection("characters").document(player_id)
    doc_ref.set(document_data=current_data)
    return current_data


@app.get("/api/characters")
async def list_characters_for_user(user=Depends(get_user_token)):
    player_id = user["uid"]
    # Explicitly defining types, because Google's Python SDK is not fully typed for some fucking reason (??)
    doc_ref: DocumentReference = db.collection("characters").document(player_id)
    doc: DocumentSnapshot = doc_ref.get()
    return {"user": doc.to_dict()}
