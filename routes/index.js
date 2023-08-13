import { Router } from "express"
const router = Router()

import * as controller from '../controllers/index.js'


router.route('/createuser').post(controller.authorizeUser)
router.route('/getallusers').get(controller.getAllUsers)
router.route('/getuser').get(controller.getUser)
router.route('/updateuser').put(controller.updateUser)
router.route('/deleteuser').delete(controller.deleteUser)


router.route('/startchain').get()


export { router }