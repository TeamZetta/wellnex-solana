import { Router } from "express"
const router = Router()

import * as controller from '../controllers/index.js'


router.route('/auth').post(controller.authorizeUser)
router.route('/getuser').get(controller.getUser)


export { router }