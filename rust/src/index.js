import rtData from './RT.json' assert { type: 'json' }

// JS原生处理
function main() {
  const { column, row, layNum, rowPixelSpacing, columnPixelSpacing, thickness, data } = rtData

  console.log(column, row, layNum, rowPixelSpacing, columnPixelSpacing, thickness, data);
}

main()