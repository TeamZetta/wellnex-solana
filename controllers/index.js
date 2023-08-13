import User from "../models/user.model.js"


export async function authorizeUser(req, res) {
    const { name, age, weight, height, email, gender, photo } = req.body

    const user = await User.findOne({ email })

    if (user) {
        return res.status(200).json({ user, msg: 'User already exists' })
    }
    else {
        try {
            const user = await User.create({ name, age, weight, height, email, gender, photo })

            return res.status(201).json({ user, msg: 'User created' })
        }
        catch (err) {
            return res.status(500).json(err)
        }
    }
}


export async function getAllUsers(req, res) {
    try {
        const users = await User.find()

        return res.status(200).json(users)
    }
    catch (err) {
        return res.status(500).json(err)
    }
}



export async function getUser(req, res) {
    const { email } = req.query
    try {
        const user = await User.findOne({ email })

        return res.status(200).json(user)
    }
    catch (err) {
        return res.status(500).json(err)
    }
}

export async function updateUser(req, res) {
    const { email } = req.query

    try {
        const user = await User.findOneAndUpdate({ email },
            {
                $set: req.body
            },
            { new: true }
        )
        return res.status(200).json(user)
    }
    catch (err) {
        return res.status(500).json(err)
    }
}


export async function deleteUser(req, res) {
    const { email } = req.query

    try {
        await User.findOneAndDelete({ email })
        return res.status(200).json('Account deleted')
    }
    catch (err) {
        return res.status(500).json(err)
    }
}