import mongoose from "mongoose"

const UserSchema = new mongoose.Schema({
    name: {
        type: String,
        required: [true, 'Name required']
    },
    age: {
        type: Number,
        required: [true, 'Age required']
    },
    weight: {
        type: Number,
        required: [true, 'Weight required']
    },
    height: {
        type: Number,
        required: [true, 'Height required']
    },
    email: {
        type: String,
        unique: [true, 'User already exists'],
        required: [true, 'Email required']
    },
    gender: {
        type: String,
        enum: ['Male', 'Female'],
        required: true
    },
    photo: { type: String },
},
    {
        toJSON: {
            transform(doc, ret) {
                delete ret.__v
                delete ret.createdAt
                delete ret.updatedAt
            }
        },
        timestamps: true
    }
)

const User = mongoose.model('User', UserSchema)
export default User