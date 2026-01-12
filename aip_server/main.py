from models import Text

from fastapi import FastAPI, HTTPException, Query, Body
import agents
from dotenv import load_dotenv
from openai import AsyncOpenAI
from agents import Agent, Runner, function_tool

load_dotenv()

# model
client = AsyncOpenAI(base_url="https://api.groq.com/openai/v1")

agents.set_default_openai_client(client)
agents.set_tracing_disabled(True)

agent = Agent(
    name="Agent",
    # tools=[],
    # instructions=""
    model="llama-3.1-8b-instant",
)

# server
app = FastAPI()


@app.get("/")
def home():
    return {"message": "Server working"}


@app.post("/prompt")
async def summarize(text: Text):
    prompt = text.text
    result = await Runner.run(agent, prompt)
    return {"response": result.final_output}
