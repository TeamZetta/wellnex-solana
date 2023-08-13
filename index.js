import express from "express"
import dotenv from 'dotenv'
import cors from 'cors'
import mongoose from 'mongoose'
import { router as apiRouter } from './routes/index.js'

const app = express()

dotenv.config()


app.use(cors())
app.use(express.json({ limit: '5mb' }))


app.get('/', (req, res) => {
    res.json('Hello from WellNex API')
})


app.use('/api/v1', apiRouter)



const port = process.env.PORT || 5000
mongoose.connect(process.env.MONGO_URI, {
    useNewUrlParser: true,
    useUnifiedTopology: true
})
    .then((conn) => {
        console.log(`Database Connected : ${conn.connection.host}`)
        app.listen(port, () => {
            console.log(`Server PORT : ${port}`)
        })
    })
