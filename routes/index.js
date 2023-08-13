import { Router } from "express"
const router = Router()

import * as controller from '../controllers/index.js'


router.route('/createuser').post(controller.authorizeUser)
router.route('/getuser').get(controller.getUser)
router.route('/updateuser').put(controller.updateUser)
router.route('/deleteuser').delete(controller.deleteUser)


export { router }