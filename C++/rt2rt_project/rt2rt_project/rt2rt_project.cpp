#include <fstream>
#include "..\cJSON\cJSON.h"
#include <vtkActor.h>
#include <vtkLineSource.h>
#include <vtkNamedColors.h>
#include <vtkPolyData.h>
#include <vtkPolyDataMapper.h>
#include <vtkProperty.h>
#include <vtkRenderer.h>
#include <vtkRenderWindow.h>
#include <vtkRenderWindowInteractor.h>
#include <vtkNew.h>
#include <vtkInteractorStyleTrackballCamera.h>


void Point2Pixel(int& px, int& py, const double x, const double y,
	const double rowPixelSpacing, const double columnPixelSpacing)
{
	px = x / rowPixelSpacing;
	py = y / columnPixelSpacing;
}

int main(int argcount, char* argv[])
{
	std::string exePath;
	if (argcount > 0)
		exePath = argv[0];
	std::ifstream filestream("");
	exePath = exePath.substr(0, exePath.find_last_of('\\'));
	std::string filename = exePath + "\\..\\..\\..\\..\\RT.json";
	filestream.open(filename, std::ios::in);
	std::string input_data;
	std::string temp;
	if (filestream.is_open())
	{
		while(filestream >> temp)
		 input_data += temp;
	}
	filestream.close();

	cJSON* input_json = cJSON_Parse(input_data.c_str());

	int column = 512;
	int row = 512;
	int layNum = 126;
	double rowPixelSpacing = 0.9765625;
	double columnPixelSpacing = 0.9765625;
	double thickness = 3;
	cJSON* data_json = cJSON_GetObjectItem(input_json, "data");
	cJSON* edge_cords3 = cJSON_GetObjectItem(cJSON_GetObjectItem(data_json, "3"),
		"edgeCoords");
	cJSON* points3_json = cJSON_GetArrayItem(edge_cords3, 0);
	int points_size = cJSON_GetArraySize(points3_json);
	std::vector<std::pair<int, int>> points_list;
	points_list.resize(points_size);
	int px = 0;
	int py = 0;
	double temp_x = 0.0;
	double temp_y = 0.0;
	for (int i = 0; i < points_size; ++i)
	{
		cJSON* child_json = cJSON_GetArrayItem(points3_json, i);
		temp_x = cJSON_GetNumberValue(cJSON_GetObjectItem(child_json, "x"));
		temp_y = cJSON_GetNumberValue(cJSON_GetObjectItem(child_json, "y"));
		Point2Pixel(px, py, temp_x, temp_y, rowPixelSpacing, columnPixelSpacing);
		points_list[i] = std::make_pair(px, py);
	}


	// Create two points, P0 and P1
	double p0[3] = { 1.0, 0.0, 0.0 };
	double p1[3] = { 0.0, 1.0, 0.0 };

	vtkNew<vtkLineSource> lineSource;
	lineSource->SetPoint1(p0);
	lineSource->SetPoint2(p1);

	// Visualize
	vtkNew<vtkNamedColors> colors;

	vtkNew<vtkPolyDataMapper> mapper;
	mapper->SetInputConnection(lineSource->GetOutputPort());
	vtkNew<vtkActor> actor;
	actor->SetMapper(mapper);
	actor->GetProperty()->SetLineWidth(4);
	actor->GetProperty()->SetColor(colors->GetColor3d("Peacock").GetData());

	vtkNew<vtkRenderer> renderer;
	vtkNew<vtkRenderWindow> renderWindow;
	renderWindow->AddRenderer(renderer);
	renderWindow->SetWindowName("Line");
	vtkNew<vtkRenderWindowInteractor> renderWindowInteractor;
	renderWindowInteractor->SetRenderWindow(renderWindow);

	renderer->SetBackground(colors->GetColor3d("Silver").GetData());
	renderer->AddActor(actor);

	renderWindow->Render();
	renderWindowInteractor->Start();

	cJSON_Delete(input_json);
	input_json = nullptr;
	return EXIT_SUCCESS;
}

