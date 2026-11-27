import hasRole from './permission/hasRole.js'
import hasPermi from './permission/hasPermi.js'
import copyText from './common/copyText.js'
import fixedTableHeight from "./common/fixedTableHeight.js";

export default function directive(app){
  app.directive('hasRole', hasRole)
  app.directive('hasPermi', hasPermi)
  app.directive('copyText', copyText)
  app.directive('fixedTableHeight', fixedTableHeight)
}