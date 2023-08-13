import express from "express"
import dotenv from 'dotenv'


dotenv.config()
const app = express()
app.use(express.json({ limit: '5mb' }))

app.get('/', (req, res) => {
    res.json('Hello Route Working!!')
})


const port = process.env.PORT || 5000
app.listen(port, () => {
    console.log(`Server PORT : ${port}`)
})