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
#include <vtkPolyLine.h>
#include <vtkNamedColors.h>

vtkNew< vtkNamedColors> colors;


void Point2Pixel(int& px, int& py, const double x, const double y,
	const double rowPixelSpacing, const double columnPixelSpacing)
{
	px = x / rowPixelSpacing;
	py = y / columnPixelSpacing;
}



void DrawMultiLine(vtkNew<vtkRenderer>& input_render,
	const std::vector<std::tuple<int, int, int>>& input_points)
{
	vtkNew<vtkPoints> pointsForline;
	for (auto& point : input_points)
	{
		pointsForline->InsertNextPoint(std::get<0>(point), 
			std::get<1>(point), std::get<2>(point));
	}
	size_t points_count = input_points.size();
	vtkNew<vtkPolyLine> polyLine;
	polyLine->GetPointIds()->SetNumberOfIds(points_count);
	for (unsigned int i = 0; i < points_count; i++)
	{
		polyLine->GetPointIds()->SetId(i, i);
	}

	// Create a cell array to store the lines in and add the lines to it
	vtkNew<vtkCellArray> cells;
	cells->InsertNextCell(polyLine);

	// Create a polydata to store everything in
	vtkNew<vtkPolyData> polyData;

	// Add the points to the dataset
	polyData->SetPoints(pointsForline);

	// Add the lines to the dataset
	polyData->SetLines(cells);

	// Setup actor and mapper
	vtkNew<vtkPolyDataMapper> mapper;
	mapper->SetInputData(polyData);

	vtkNew<vtkActor> actor;
	actor->SetMapper(mapper);
	actor->GetProperty()->SetColor(colors->GetColor3d("Tomato").GetData());
	input_render->AddActor(actor);

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
	std::vector<std::tuple<int, int, int>> points_list;
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
		points_list[i] = std::make_tuple(px, py, 0);
	}


	vtkNew<vtkRenderer> renderer;
	vtkNew<vtkRenderWindow> renderWindow;
	renderWindow->AddRenderer(renderer);
	renderWindow->SetWindowName("Line");
	vtkNew<vtkRenderWindowInteractor> renderWindowInteractor;
	renderWindowInteractor->SetRenderWindow(renderWindow);

	renderer->SetBackground(colors->GetColor3d("Silver").GetData());

	DrawMultiLine(renderer, points_list);

	renderWindow->Render();
	renderWindowInteractor->Start();

	cJSON_Delete(input_json);
	input_json = nullptr;
	return EXIT_SUCCESS;
}

